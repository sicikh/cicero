import { Text } from "@mantine/core";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createRouter } from "@tanstack/react-router";
import { Suspense } from "react";
import { routeTree } from "./routeTree.gen";

const createRouterWithContext = () => {
  const queryClient = new QueryClient();

  return createRouter({
    routeTree,
    context: {
      queryClient: queryClient,
      // biome-ignore lint/style/noNonNullAssertion: we will set the context in the App.tsx
      auth: undefined!,
    },
    Wrap: ({ children }) => (
      <Suspense fallback={<Text>Приложение загружается...</Text>}>
        <QueryClientProvider client={queryClient}>
          {children}
        </QueryClientProvider>
      </Suspense>
    ),
  });
};
export const router = createRouterWithContext();

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
