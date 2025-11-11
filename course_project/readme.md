# E-Commerce Backend (Rust + Axum)

A simple e-commerce backend built with `Rust` using the `Axum` framework.

Frontend is implemented with `Next.js` and `Zustand` for state management.

This project was created as part of the LUT University – Rust Programming 2025–26 course.

<img width="1408" height="758" alt="Screenshot 2025-11-10 at 18 03 52" src="https://github.com/user-attachments/assets/a5330030-8363-4cb7-9755-a9d2f92eede1" />

<img width="1408" height="777" alt="Screenshot 2025-11-10 at 18 04 38" src="https://github.com/user-attachments/assets/288f8542-8a64-49d6-9517-11aaef73a487" />

<img width="304" height="661" alt="Screenshot 2025-11-10 at 18 10 55" src="https://github.com/user-attachments/assets/fa8ea7d9-99b6-4857-95bb-5513e6385a83" />

## How to use

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
