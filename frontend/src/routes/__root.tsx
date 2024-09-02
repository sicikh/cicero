import {
  Group,
  Button,
  Divider,
  Burger,
  Drawer,
  Container,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Title } from "@mantine/core";
import classes from "./__root.module.css";
import {
  Link,
  Outlet,
  createRootRouteWithContext,
  useMatchRoute,
} from "@tanstack/react-router";
import type { QueryClient } from "@tanstack/react-query";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import "@mantine/core/styles.css";
import "@mantine/dates/styles.css";
import type { AuthState } from "../hooks/AuthProvider.tsx";

const links = [
  { link: "/", label: "Главная" },
  { link: "/templates", label: "Конструктор" },
];

const linksReg = [
  { link: "/login", label: "Логин" },
  { link: "/register", label: "Регистрация" },
];

const Page: React.FC = () => {
  const [drawerOpened, { toggle: toggleDrawer, close: closeDrawer }] =
    useDisclosure(false);

  const matchRoute = useMatchRoute();
  const hideNavRoutes = ["/login", "/register"];
  const matchedNoNavRoutes = hideNavRoutes.some((route) =>
    matchRoute({ to: route }),
  );

  const items = links.map((link) => (
    <Link key={link.link} search={{}} to={link.link} className={classes.link}>
      {link.label}
    </Link>
  ));
  const itemsReg = linksReg.map((link_reg) => (
    <Link key={link_reg.link} to={link_reg.link} className={classes.linkBut}>
      <Button
        className={classes.ButtonLink}
        size="lg"
        variant="outline"
        color="#495057"
        radius="lg"
      >
        {link_reg.label}
      </Button>
    </Link>
  ));

  return (
    <>
      {!matchedNoNavRoutes && (
        <header>
          <Container size="2xl" className={classes.inner}>
            <Link to={"/"}>
              <div className={classes.logo}>
                <Title size="h1">Cicero</Title>
              </div>
            </Link>
            <Group gap={2} visibleFrom="sm" className={classes.links}>
              <div className={classes.linksItem}>{items}</div>
              <div className={classes.linksReg}>{itemsReg}</div>
            </Group>
            <Burger
              opened={drawerOpened}
              onClick={toggleDrawer}
              hiddenFrom="sm"
            />
          </Container>

          <Drawer
            opened={drawerOpened}
            onClose={closeDrawer}
            size="50%"
            position="left"
            padding="md"
            title="Cicero"
            hiddenFrom="sm"
            color="#DEE2E6"
          >
            <Divider my="sm" />
            <div className={classes.smallbar}>{items}</div>
            <Divider my="sm" />
            <div className={classes.bigbar}>{itemsReg}</div>
          </Drawer>
        </header>
      )}
      <Outlet />
      <TanStackRouterDevtools />
    </>
  );
};

interface RouterContext {
  queryClient: QueryClient;
  auth: AuthState;
}

export const Route = createRootRouteWithContext<RouterContext>()({
  component: Page,
});
