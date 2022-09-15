$env:GOOS = "linux"
$env:GOARCH = "amd64"
$env:CGO_ENABLED = "0"
~\Go\Bin\build-lambda-zip.exe -o .\lambda.zip .\bootstrap

