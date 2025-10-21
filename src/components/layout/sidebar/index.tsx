import { Link } from "@tanstack/react-router";

export function Sidebar() {
	return (
		<aside>
			<nav>
				<ul>
					<li>
						<Link
							to="/home"
							activeProps={{
								className: "bg-gray-200 font-semibold",
							}}
						>
							Home
						</Link>
					</li>
					<li>
						<Link
							to="/about"
							activeProps={{
								className: "bg-gray-200 font-semibold",
							}}
						>
							About
						</Link>
					</li>
				</ul>
			</nav>
		</aside>
	);
}
