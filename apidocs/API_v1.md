# API DOCS:

## Note: i havent mentioned the url cause typing the url everysingle time is would be very message so i have dropped the global variable of the url 😅:
```dart
String url = 'http://localhost:3000';
```

# **Auth**:
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

## Token Validator:
- Type: ```POST``` 
- Endpoint: ```{url}/tokenIsValid```
### Format:
```json
{
    "token": "userIdToken" // send the user id token here.
}
```

### Response:
```json
false or true.
```


# **Admin Functionalities** :
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

## Get Product:
- getting products which is wandering in the backend
- Type: ```GET```
- ```{url}/admin/get-products``` endpoint url to get all products which is available.

## Delete Product:
- delete products which is current available.
- Type ```POST```
- ```{url}/admin/delete-product``` endpoint url to delete products.

## Get Current Orders:
- get the current orders which is ordered by the user.
- Type ```GET```
- ```{url}/admin/get-orders``` endpoint url to retrive the ordered information done by the users.

## Change In Order Status:
- changing the order status, will help the user updating the status of the order(example: updating that the packages has left the facility)
- Type ```POST```
- Endpoint ```{url}/admin/change-order-status``` endpoint url to change the order status.

## Analytics
- if your an admin there would be some cool features right? here is one of the cool feature, you can view the analytics about different platform that your product sold.
- Type ```GET```
- Endpoint ```{url}/admin/analytics```

# **Product Functionalities**:
### let's see the product functionalities[getting the products, searching products, rate the product] & of-course deal-of-the-day feature also there.
## Products:
- Type: ```GET```
- Endpoint: ```{url}/api/products/```

## Search Product:
- Type: ```GET```
- Endpoint: ```{url}/api/products/search/:name``` 

## Rate Product
- Type: ```POST```
- Endpoint: ```{url}/api/rate-product/```

## Deal Of The Day(Special Feature):
- who don't loves deal-of-the-day that the whole nation loves.
- Type: ```GET```
- Endpoint: ```{url}/api/deal-of-day```

# And Finally **User Functionalities**:
## Add To Cart:
- Type: ```POST```
- Endpoint: ```{url}/api/add-to-cart```

## Remove From Cart:
- Type: ```DELETE```
- Endpoint: ```{url}/api/remove-from-cart/:id```

## Save User Address(Home Address) it would be helpfull for the user so that he dont want to enter his address (memorizing the address is a big task 😅):
- Type: ```POST```
- Endpoint: ```{url}/api/save-user-address```

## Order:
- Type: ```POST```
- Endpoint: ```{url}/api/order``` = helps to place the Order.

## My Orders:
- view the orders which is placed by you.
- Type: ```GET```
- Endpoint: ```{url}/api/orders/me```