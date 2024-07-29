include ../../.env

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

deploy-zip:
	cargo lambda build --release
	cargo lambda deploy \
		--profile transcribe-lambda-dev \
		--region ${AWS_DEFAULT_REGION} \
		--iam-role arn:aws:iam::${AWS_ACCT_ID}:role/listener-fxn-role \
		--env-vars STATE_MACHINE_ARN=${STATE_MACHINE_ARN}