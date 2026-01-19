# Jumper Frontend

React + TypeScript frontend for the Jumper Wake-on-LAN application.

## Setup

```bash
# Install dependencies
cd frontend
pnpm install
```

## Development

Run the frontend dev server:

```bash
cd frontend
pnpm dev
```

This starts the Vite dev server on http://localhost:5173.

Make sure your Rust backend is also running on port 3000:

```bash
cargo run
```

The frontend will proxy API requests to the backend automatically.

## Building for Production

```bash
cd frontend
pnpm build
```

This creates optimized production files in `../static/dist/`.

Then run the backend:

```bash
cargo run
```

The backend will serve the built files from `static/dist/` on http://localhost:3000.

## Available Scripts

- `pnpm dev` - Start development server
- `pnpm build` - Build for production
- `pnpm preview` - Preview production build
- `pnpm lint` - Run ESLint

## Project Structure

```
src/
├── api/              # API client functions
├── components/       # React components with CSS Modules
├── hooks/           # Custom React hooks (TanStack Query)
├── styles/          # Global CSS styles
├── types/           # TypeScript type definitions
├── App.tsx          # Root component
└── main.tsx         # Entry point
```

## Technologies

- React
- TypeScript
- Vite
- TanStack Query
- React Hook Form
- CSS Modules
- ESLint + Prettier
