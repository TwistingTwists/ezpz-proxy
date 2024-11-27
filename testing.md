1. generate local ssl self-signed certificates (make sure to fill all fields with some value)

```sh
openssl req -new -newkey rsa:4096 -x509 -sha256 -days 3650 -nodes -out provab-certi.crt -keyout provab-key.key
```

2. up the python server (has logging in it)


```sh
python python_server.py
```

3. make curl request

```sh
curl --cacert ./provab-certi.crt -vvv https://localhost:8043
```