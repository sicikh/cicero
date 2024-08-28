import { Text } from "@mantine/core";
import type { QueryClient } from "@tanstack/react-query";
import {
  Link,
  Outlet,
  createRootRouteWithContext,
} from "@tanstack/react-router";
// import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import type React from "react";
import styles from "./__root.module.css";
import "@mantine/dates/styles.css";

const Page: React.FC = () => (
  <div className={styles.Container}>
    <div className={styles.Header}>
      <Link to={"/"}>
        <Text className="text-2xl">Cicero</Text>
      </Link>
    </div>
    <Outlet />
    {/*<TanStackRouterDevtools />*/}
  </div>
);

interface RouterContext {
  queryClient: QueryClient;
}

export const Route = createRootRouteWithContext<RouterContext>()({
  component: Page,
});
