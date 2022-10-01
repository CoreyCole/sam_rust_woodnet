# rust lambda
### dependencies
- rust, cargo
- [cargo lambda](https://www.cargo-lambda.info/guide/getting-started.html)
- AWS Serverless Application Model (SAM) [CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
- [docker](https://www.docker.com/products/docker-desktop/)
- [llvm](https://github.com/llvm/llvm-project/releases)

### commands
```
# build lambda.zip for arm64 lambda AL2 runtime
make build

# test locally with that build
sam local start-api

# make request to localhost lambda
curl localhost:3000 --data-binary @test\data\SignUp.json
```

