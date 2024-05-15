create-canisters:
	@dfx canister create --all

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
	@echo ""
	@echo "Deployed issuer canister"
	@echo ""
	@echo "Web interface: \033[1;93mhttp://$$(dfx canister id issuer).localhost:4943\033[0m"

deploy-demo-app:
	@dfx deploy demo_app

start-demo-app:
	@npm --workspace demo_app run start

deploy-all: create-canisters deploy-internet-identity deploy-issuer-backend deploy-issuer-frontend deploy-demo-app

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
