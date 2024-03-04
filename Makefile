create-canisters:
	@dfx canister create --all

deploy-issuer-backend:
	@dfx deploy issuer_backend

deploy-issuer-frontend:
	@dfx deploy issuer_frontend

deploy-demo-app:
	@dfx deploy demo_app

deploy-all: 
	@$(MAKE) create-canisters
	@$(MAKE) deploy-issuer-backend
	@$(MAKE) deploy-issuer-frontend
	@$(MAKE) deploy-demo-app
	@echo ""
	@echo "Deployment Complete."
	@echo ""
	@echo "Access issuer frontend: \033[1;93mhttp://$$(dfx canister id issuer_frontend).localhost:4943\033[0m"
	@echo "Access demo app: \033[1;93mhttp://$$(dfx canister id demo_app).localhost:4943\033[0m"

clean:
	rm -rf .dfx
	rm -rf node_modules
	rm -rf packages/declarations
	rm -rf packages/demo_app/dist
	rm -rf packages/demo_app/node_modules
	rm -rf packages/issuer_frontend/dist
	rm -rf packages/issuer_frontend/node_modules
	rm -f .env
	cargo clean
