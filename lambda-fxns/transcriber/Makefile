include ../../.env
SM_DEFINITION := $(shell jq -c . < statemachine.json)

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

build-image:
	docker build --platform linux/amd64 -t transcriber .

ecr-login:
	aws ecr get-login-password --profile transcribe-lambda-dev --region ${AWS_DEFAULT_REGION} | docker login --username AWS --password-stdin ${AWS_ACCT_ID}.dkr.ecr.${AWS_DEFAULT_REGION}.amazonaws.com

ecr-repo:
	aws ecr --profile transcribe-lambda-dev --region ${AWS_DEFAULT_REGION} create-repository --repository-name transcriber > /dev/null

ecr-push:
	docker tag transcriber:latest ${AWS_ACCT_ID}.dkr.ecr.${AWS_DEFAULT_REGION}.amazonaws.com/transcriber:latest
	docker push ${AWS_ACCT_ID}.dkr.ecr.${AWS_DEFAULT_REGION}.amazonaws.com/transcriber:latest

deploy-lambda:
	aws lambda create-function \
		--profile transcribe-lambda-dev \
		--region ${AWS_DEFAULT_REGION} \
		--function-name transcriber \
		--timeout 90 \
		--package-type Image \
		--code ImageUri=${AWS_ACCT_ID}.dkr.ecr.${AWS_DEFAULT_REGION}.amazonaws.com/transcriber:latest \
		--role arn:aws:iam::${AWS_ACCT_ID}:role/transcriber-fxn-role \
		> /dev/null

update-lambda:
	aws lambda update-function-code \
		--profile transcribe-lambda-dev \
		--region ${AWS_DEFAULT_REGION} \
		--function-name transcriber \
		--image-uri ${AWS_ACCT_ID}.dkr.ecr.${AWS_DEFAULT_REGION}.amazonaws.com/transcriber:latest \
		> /dev/null

install-emulator:
	# https://github.com/aws/aws-lambda-runtime-interface-emulator
	mkdir -p ~/.aws-lambda-rie && \
    curl -Lo ~/.aws-lambda-rie/aws-lambda-rie https://github.com/aws/aws-lambda-runtime-interface-emulator/releases/latest/download/aws-lambda-rie && \
    chmod +x ~/.aws-lambda-rie/aws-lambda-rie

local-container:
	docker run --env-file .env -d -p 9000:8080 --entrypoint /usr/local/bin/aws-lambda-rie $transcriber:latest transcriber