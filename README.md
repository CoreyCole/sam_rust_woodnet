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

## db
```
# run local surrealdb
surreal start --log debug --user root --pass root memory
```
```
# create user
curl localhost:8000/sql --data @test\db\create_user.sql --header "Content-Type: application/json" --header "NS: test" --header "DB: test" --user "root:root"
```
```
[{
    "time":"500.4µs",
    "status":"OK",
    "result": [
        {"created_at":"2022-10-01T22:42:36.304790800Z","id":"user:e0bbki44ki0a8qfc2m5h","name":"Corey"}
    ]
}]
```
```
# get users
curl localhost:8000/sql --data @test\db\get_users.sql --header "Content-Type: application/json" --header "NS: test" --header "DB: test" --user "root:root"
```
```
[{
    "time":"126µs",
    "status":"OK",
    "result": [
        {"created_at":"2022-10-01T22:42:36.304790800Z","id":"user:e0bbki44ki0a8qfc2m5h","name":"Corey"}
    ]
}]
```
