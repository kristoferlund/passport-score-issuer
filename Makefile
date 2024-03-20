create-canisters:
	@dfx canister create --all

deploy-internet-identity:
	@dfx deploy internet_identity

deploy-issuer:
	@npm run build --workspace issuer_frontend
# At the time of writing dfx outputs incorrect JSON with dfx ping (commas between object
# entries are missing).
# In order to read the root key we grab the array from the '"root_key": [...]' bit, the brackets
# to match what candid expects ({}), replace the commas between array entries to match
# what candid expects (semicolon) and annotate the numbers with their type (otherwise dfx assumes 'nat'
# instead of 'nat8').
	$(eval export ROOT_KEY=$(shell dfx ping \
		| sed -n 's/.*"root_key": \[\(.*\)\].*/{\1}/p' \
		| sed 's/\([0-9][0-9]*\)/\1:nat8/g' \
		| sed 's/,/;/g'))
	@dfx deploy issuer_backend --argument "( \
	    record { \
				ic_root_key_der = vec $(ROOT_KEY); \
				ii_canister_id = principal \"$$(dfx canister id internet_identity)\"; \
	    } \
	)"

deploy-demo-app:
	@dfx deploy demo_app

start-demo-app:
	@npm --workspace demo_app run start

deploy-all: create-canisters deploy-internet-identity deploy-issuer deploy-demo-app
	@echo ""
	@echo "Deployment Complete."
	@echo ""
	@echo "Issuer frontend: \033[1;93mhttp://$$(dfx canister id issuer_frontend).localhost:4943\033[0m"
	@echo "Demo app: \033[1;93mhttp://$$(dfx canister id demo_app).localhost:4943\033[0m"

clean:
	rm -rf .dfx
	rm -rf node_modules
	rm -rf packages/declarations
	rm -rf packages/demo_app/dist
	rm -rf packages/demo_app/node_modules
	rm -rf packages/issuer_frontend/dist
	rm -rf packages/issuer_frontend/node_modules
	rm -rf declarations
	rm -f .env
	cargo clean
