import { useAuth as useIdentityKitAuth } from "@nfid/identitykit/react";
import { useEffect, useState } from "react";

export function useAuth() {
  const { user, connect, disconnect, isConnecting } = useIdentityKitAuth();
  const [isSignedIn, setIsSignedIn] = useState(false);

  useEffect(() => {
    // Update local state when user authentication changes
    setIsSignedIn(!!user);
  }, [user]);

  const signIn = async () => {
    try {
      await connect();
    } catch (error) {
      console.error("Sign in failed:", error);
      throw error;
    }
  };

  const signOut = async () => {
    try {
      await disconnect();
    } catch (error) {
      console.error("Sign out failed:", error);
      throw error;
    }
  };

  return {
    isSignedIn,
    user,
    signIn,
    signOut,
    isConnecting,
  };
}
