create-canisters:
	@dfx canister create --all

deploy-issuer-backend:
	@dfx deploy issuer_backend

deploy-issuer-frontend:
	@dfx deploy issuer_frontend

start-issuer-frontend:
	@npm --workspace issuer_frontend run start

deploy-demo-app:
	@dfx deploy demo_app

start-demo-app:
	@npm --workspace demo_app run start

deploy-all: 
	@dfx deploy
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
