# EDC Finc

A lightweight, local‑first personal finance desktop app built with Tauri, inspired by Firefly III.

EDC Finc aims to bring the clarity and discipline of Firefly III into a fast, private, and portable desktop experience powered by Rust + Web tech.

## Motivation
Why this app exists:
- Privacy by default: All your data lives on your machine. No cloud account is required.
- Local‑first speed: Tauri uses Rust under the hood, delivering a snappy desktop UX with a tiny footprint.
- Inspired by Firefly III: Firefly III proves that personal finance apps can be powerful, transparent, and user‑respecting. EDC Finc draws from its concepts while focusing on a streamlined desktop workflow.
- Focused workflows: Simple daily flows for logging transactions, reviewing budgets, and tracking balances without unnecessary distractions.
- Portability and ownership: Easy backups and export; you own your data.
- Cost control: No subscriptions.
- Learning and exploration: This project is also a space to explore modern desktop tooling (Tauri, Vite, React, TypeScript) and robust data models.

## What it is (and isn’t)
- Is: A personal finance manager for the desktop, optimized for offline use and individual control.
- Isn’t: A hosted service, an accounting system for businesses, or a full clone of Firefly III.

## Core Ideas (Early Roadmap)
- Accounts, categories, and tags
- Income, expense, and transfer transactions
- Budgets and period summaries
- Simple import/export (CSV first)
- Fast local database (SQLite) with safe migrations
- Clean, keyboard‑friendly UI
- Optional encryption at rest (planned)

## Tech Stack
- Tauri (Rust) for the desktop shell and native capabilities
- React + TypeScript + Vite for the UI
- Tailwind CSS for styling

## Getting Started
Prerequisites:
- Rust and Cargo (for Tauri)
- Node.js (or Bun) and a package manager (npm, pnpm, or bun)

Install dependencies:
- npm install
  or
- bun install

Run in development:
- npm run tauri dev
  or
- bun run tauri dev

Build a desktop binary:
- npm run tauri build
  or
- bun run tauri build

Note: First build may take a bit while Rust dependencies compile.

## Project Status
Early scaffold/experiment. Expect rapid changes and breaking updates while core features solidify.

## Contributing
Ideas, feedback, and small PRs are welcome! Please open an issue to discuss significant changes.

## Acknowledgements
- Firefly III (https://www.firefly-iii.org/) for the inspiration, concepts, and its great community around personal finance.
- The Tauri team and ecosystem for making secure, lightweight desktop apps possible.

## License
TBD. Until specified, assume all rights reserved for early development. If licensing is critical for you, open an issue to discuss.
