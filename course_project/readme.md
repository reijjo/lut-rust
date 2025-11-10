# LUT University - Course Project Rust Programming 2025-26

Backend Application for e-commerce site done with `Axum` framework

- Frontend done with `NextJS`

## How to use

## Technologies

### Backend

`Rust` backend with `Axum` framework

### Frontend

Frontend is done with `NextJS`

#### API

Using <https://fakestoreapi.com/> for the products

##### Routes

Product route:

- `GET /products` for fetching all the products
- `GET /products/:id` for fetching single product

Cart route:

- `GET /cart` for the cart
- `POST /cart` adding item to cart
- `DELETE /cart` clearing the cart
- `PATCH /cart/:id` updating item in the cart
- `DELETE /cart/:id` deleting item in the cart
