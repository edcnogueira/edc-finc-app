import { createRootRoute, Outlet } from "@tanstack/react-router";
import { Header } from "../components/layout/header";
import { Sidebar } from "../components/layout/sidebar";

const RootLayout = () => (
	<div className="flex min-h-screen flex-col">
		<Header />
		<div className="flex flex-1">
			<Sidebar />
			<main className="flex-1 p-4">
				<Outlet />
			</main>
		</div>
	</div>
);

export const Route = createRootRoute({ component: RootLayout });
