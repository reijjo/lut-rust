# E-Commerce Backend (Rust + Axum)

A simple e-commerce backend built with `Rust` using the `Axum` framework.

Frontend is implemented with `Next.js` and `Zustand` for state management.

This project was created as part of the LUT University – Rust Programming 2025–26 course.

## How to use

<!-- ### Live
Try it here: <http://something.vercel.com>
- Frontend deployed with `Vercel`
- Backend deployed with `Render` -->

### Run Locally

#### 1. Rename environment files

In both the `client` and `server` folders, rename `.env.example` → `.env`.

#### 2. Start the frontend

```bash
cd client
npm run build-dev
```

Runs at: <http://localhost:3000/>

#### 3. Start the backend

```bash
cd server
cargo run
```

Runs at: <http://localhost:3001/>

## Technologies

### Backend

- `Rust` + `Axum` for HTTP server
- `Arc<Mutex<Cart>>` simulate a database

### Frontend

- `NextJS` for UI
- `Zustand` for state management
- `Fake Store API` as data source → <https://fakestoreapi.com/>

## API Routes

### Products

| Method | Route           | Description          |
| :----- | :-------------- | :------------------- |
| `GET`  | `/products`     | Fetch all products   |
| `GET`  | `/products/:id` | Fetch single product |

### Cart

| Method   | Route       | Description           |
| :------- | :---------- | :-------------------- |
| `GET`    | `/cart`     | Get current cart      |
| `POST`   | `/cart`     | Add item to cart      |
| `DELETE` | `/cart`     | Clear entire cart     |
| `PATCH`  | `/cart/:id` | Update item quantity  |
| `DELETE` | `/cart/:id` | Remove item from cart |

## What I learned

- Setting up a **Rust backend** with the **Axum** framework
- Handling **shared mutable state** safely with `Arc<Mutex<>>`
