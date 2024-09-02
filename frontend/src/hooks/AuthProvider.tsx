import {useContext, createContext, useState} from "react";
import { LoginApi } from "../routes/login/-api/login.api.ts";
import type { LoginDto } from "../routes/login/-api/dtos/Login.dto.ts";
import { useQuery } from "@tanstack/react-query";
import type { LoginResponseDto } from "../routes/login/-api/dtos/LoginResponse.dto.ts";

export type AuthState = {
  user: LoginResponseDto;
  token: string;
  login: (data: LoginDto) => Promise<{ success: boolean; message?: string }>;
  logout: () => void;
  isAuthenticated: boolean;
};

const AuthContext = createContext<AuthState | undefined>(undefined);

const AuthProvider = ({ children }) => {
  const [user, setUser] = useState<LoginResponseDto>();
  const [token, setToken] = useState<string | null>(
    localStorage.getItem("token"),
  );
  const isAuthenticated = !!token;

  const login = async (data: LoginDto) => {
    try {
      const res = useQuery(LoginApi.login(data));

      if (res.data) {
        setUser(res.data);
        setToken(res.data.token);
        localStorage.setItem("token", res.data.token);
        return { success: true };
      }

      return { success: false, message: res.error.message };
    } catch (e) {
      return { success: false, message: e.message };
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
