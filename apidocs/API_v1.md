# API DOCS:

# Auth:
### Authentication + Validation:

## Signup:
- Type: ```POST```
- Endpoint: ```{url}/api/signup```
### Format:
```json
{
    "name": "BulkMan",
    "email": "bulkman19@gmail.com",
    "password": "bulkman"
}
```
### Response:
```json
{
    "name": "BulkMan",
    "email": "bulkman19@gmail.com",
    "password": "$2a$08$d87CIlLrgkTOaXm8rHbleO9vFd634DD.0Bf.XkXcg6LVdc0hUD4RK", // password will be encrypted.
    "address": "",
    "type": "user",
    "_id": "649d2efce5db2be3233ebd07",
    "cart": [],
    "__v": 0
}
```

## Signin
- Type: ```POST``` 
- Endpoint: ```{url}/api/signin```
### Format:
```json
{
    "email": "bulkman19@gmail.com",
    "password": "bulkman"
}
```

### Response:
```json
{
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY0OWQyZWZjZTVkYjJiZTMyMzNlYmQwNyIsImlhdCI6MTY4ODAyMjkzN30.YE1REHjQoRAp8ef6XKlRM0SBg6Ev3k7W3sIbe2FCwdQ",
    "_id": "649d2efce5db2be3233ebd07",
    "name": "BulkMan",
    "email": "bulkman19@gmail.com",
    "password": "$2a$08$d87CIlLrgkTOaXm8rHbleO9vFd634DD.0Bf.XkXcg6LVdc0hUD4RK",
    "address": "",
    "type": "user",
    "cart": [],
    "__v": 0
}
```

# Admin Functionalities:
### there are n number of admin features which has been added to our enital shopping app.

## Add Product:
- adding product as admin.
- Type: ```POST```
- ```{url}/admin/add-product``` is the endpoint url to add product as admin.
### Format:
```json
{
    "name": "Product One",
    "description": "Description Of Product One",
    "images": "imageurl",
    "quantity": "100",
    "price": "30$",
    "category": "Fashion"
}
```
### RESPONSE:
```

```