create-canisters:
	@dfx canister create --all

build-issuer-frontend:
	@dfx generate issuer
	@npm i
	@npm run build --workspace issuer_frontend

build-demo-app-frontend:
	@dfx generate issuer
	@npm i
	@npm run build --workspace demo_app_frontend

deploy-internet-identity:
	@dfx deploy internet_identity

deploy-issuer:
	@dfx generate issuer
	@npm i
	@npm run build --workspace issuer_frontend
	# Use jq to extract and transform the root_key directly from the output of dfx ping.
	# jq will be used to parse JSON output, format the root_key array, and transform it into the required format.
	$(eval export ROOT_KEY=$(shell dfx ping \
		| jq -r '"{" + (.root_key | map(tostring + ":nat8") | join(";")) + "}"'))
	@dfx deploy issuer --argument "( \
	    record { \
				ic_root_key_der = vec $(ROOT_KEY); \
				ii_canister_id = principal \"$$(dfx canister id internet_identity)\"; \
	    } \
	)"
	@find . -name '.DS_Store' -delete
	@candid-extractor target/wasm32-unknown-unknown/release/issuer_backend.wasm > packages/issuer_backend/issuer_backend.did

deploy-demo-app:
	@dfx generate issuer
	@npm i
	@npm run build --workspace demo_app_frontend
	# Use jq to extract and transform the root_key directly from the output of dfx ping.
	# jq will be used to parse JSON output, format the root_key array, and transform it into the required format.
	$(eval export ROOT_KEY=$(shell dfx ping \
		| jq -r '"{" + (.root_key | map(tostring + ":nat8") | join(";")) + "}"'))
	@dfx deploy demo_app --argument "( \
	    record { \
				ic_root_key_der = vec $(ROOT_KEY); \
				ii_canister_id = principal \"$$(dfx canister id internet_identity)\"; \
				issuer_canister_id = principal \"$$(dfx canister id issuer)\"; \
	    } \
	)"
	@find . -name '.DS_Store' -delete
	@candid-extractor target/wasm32-unknown-unknown/release/demo_app_backend.wasm > packages/demo_app_backend/demo_app_backend.did

run-issuer-frontend:
	@dfx generate issuer
	@npm i
	@npm --workspace issuer_frontend run start

run-demo-app-frontend:
	@dfx generate issuer
	@npm i
	@npm --workspace demo_app_frontend run start

post-deploy-message:
	@echo ""
	@echo "Deployment complete."
	@echo ""
	@echo "Issuer: \033[1;93mhttp://$$(dfx canister id issuer).localhost:4943\033[0m"
	@echo ""
	@echo "Demo app: \033[1;93mhttp://$$(dfx canister id demo_app).localhost:4943\033[0m"

deploy-all: create-canisters deploy-internet-identity deploy-issuer deploy-demo-app post-deploy-message

clean:
	rm -rf .dfx
	rm -rf node_modules
	rm -rf packages/demo_app/dist
	rm -rf packages/demo_app/declarations
	rm -rf packages/internet_identity/declarations
	rm -rf packages/issuer_backend/declarations
	rm -rf packages/issuer_frontend/dist
	rm -rf packages/issuer_frontend/declarations
	rm -rf target
	rm -f .env
	cargo clean
