# rust lambda
```
# build lambda.zip for arm64 lambda AL2 runtime
make build

# test locally with that build
sam local start-api

# make request to localhost lambda
curl localhost:3000 --data-binary @test\data1.json
```

