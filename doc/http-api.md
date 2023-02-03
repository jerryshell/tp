# HTTP API

For a detailed description of the error code, please see the code: `src/error.rs`

## *TODO* GET `/`

Get server info, usually used for server health checks.

### Response

```json
{
    "repository": "https://github.com/jerryshell/tp",
    "license": "https://choosealicense.com/licenses/agpl-3.0"
}
```

## *TODO* POST `/auth/register`

Register a new user.

### Request body

```json
{
    "email": "your@email.com",
    "password": "your_password"
}
```

### Response (success, status == 200)

```json
{
    "code": "success"
}
```

### Response (failed, status != 200)

```json
{
    "code": "register_failed",
    "message": "email already exist"
}
```

## *TODO* POST `/auth/login`

User login and get token.

### Request body

```json
{
    "email": "your@email.com",
    "password": "your_password"
}
```

### Response (success, status == 200)

```json
{
    "code": "success",
    "token": "jwt_token"
}
```

### Response (failed, status != 200)

```json
{
    "code": "login_failed",
    "message": "wrong email or password"
}
```

## *TODO* POST `/auth/update/email`

Update auth email.

Require `token` in request header.

### Request body

```json
{
    "email": "your_new_email@email.com"
}
```

### Response (success, status == 200)

```json
{
    "code": "success",
    "email": "your_new_email@email.com"
}
```

### Response (failed, status != 200)

```json
{
    "code": "invalid_token",
    "message": "invalid token"
}
```

## *TODO* POST `/auth/update/password`

Update auth password.

Require `token` in request header.

### Request body

```json
{
    "password": "your_new_password"
}
```

### Response (success, status == 200)

```json
{
    "code": "success"
}
```

## *TODO* GET `/user/profile`

Get user profile.

Require `token` in request header.

### Response (success, status == 200)

```json
{
    "id": "your_id",
    "createAt": 10000,
    "updateAt": 10000
}
```

## *TODO* GET `/link/list`

Get link list.

Require `token` in request header.

### Response (success, status == 200)

```json
{
    "code": "success",
    "data": [
        {
            "id": "link_id",
            "createAt": 10000,
            "updateAt": 10000,
            "userId": "userId",
            "targetLink": "https://examle.com"
        }
    ]
}
```

## *TODO* POST `/link/create`

Create a new link.

Require `token` in request header.

### Request

```json
{
    "id": "link_id",
    "targetLink": "https://example.com"
}
```

### Response (success, status == 200)

```json
{
    "code": "success",
    "id": "link_id",
    "targetLink": "https://example.com"
}
```

## *TODO* POST `/link/update/targetLink`

Update the target link of the specified id link.

Require `token` in request header.

### Request

```json
{
    "id": "link_id",
    "targetLink": "https://example.com"
}
```

### Response (success, status == 200)

```json
{
    "code": "success",
    "id": "link_id",
    "targetLink": "https://example.com"
}
```

## *TODO* POST `/link/delete`

Delete link.

Require `token` in request header.

### Request

```json
{
    "id": "link_id"
}
```

### Response (success, status == 200)

```json
{
    "code": "success",
    "id": "link_id",
    "targetLink": "https://example.com"
}
```

## *TODO* GET `/goto/:link_id`

302 to target link.

### Response (success, status == 302)

302 to target link.

### Response (failed, status != 302)

```json
{
    "code": "link_not_found",
    "message": "link not found"
}
```