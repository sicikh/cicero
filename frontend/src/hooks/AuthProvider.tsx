import { useQuery } from "@tanstack/react-query";
import { createContext, useContext, useState } from "react";
import type React from "react";
import type { UserDto } from "../routes/-api/dtos/User.dto.ts";
import type { LoginDto } from "../routes/login/-api/dtos/Login.dto.ts";
import { LoginApi } from "../routes/login/-api/login.api.ts";

export type AuthState = {
  user: UserDto | null;
  token: string | null;
  login: (data: LoginDto) => Promise<{ success: boolean }>;
  logout: () => void;
  isAuthenticated: boolean;
};

// biome-ignore lint/style/noNonNullAssertion: <explanation>
const AuthContext = createContext<AuthState>(undefined!);

interface AuthProviderProps {
  children: React.ReactNode;
}

const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<UserDto | null>(null);
  const [token, setToken] = useState<string | null>(
    localStorage.getItem("token"),
  );
  const isAuthenticated = !!token;

  const login = async (data: LoginDto) => {
    try {
      const res = useQuery(LoginApi.login(data));

      if (res.data) {
        setUser({
          pid: res.data.pid,
          name: res.data.name,
          email: res.data.email,
        });
        setToken(res.data.token);
        localStorage.setItem("token", res.data.token);
        return { success: true };
      }

      return { success: false };
    } catch (e) {
      return { success: false };
    }
  };

  const logout = () => {
    setUser(null);
    setToken(null);
    localStorage.removeItem("token");
  };

  return (
    <AuthContext.Provider
      value={{ user, token, login, logout, isAuthenticated }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export default AuthProvider;

export const useAuth = () => {
  return useContext(AuthContext);
};
