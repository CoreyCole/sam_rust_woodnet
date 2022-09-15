# rust lambda
### dependencies
- rust, cargo
- [cargo lambda](https://www.cargo-lambda.info/guide/getting-started.html)
- AWS Serverless Application Model (SAM) [CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
- [docker](https://www.docker.com/products/docker-desktop/)

### commands
```
# build lambda.zip for arm64 lambda AL2 runtime
make build

# test locally with that build
sam local start-api

# make request to localhost lambda
curl localhost:3000 --data-binary @test\data1.json
```

