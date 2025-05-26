import { useState, useEffect } from "react";
import { Button, Card } from "../components";
import { createBackendService } from "../services/backendService";
import { useBackendActors } from "../hooks";

interface CounterViewProps {
  onError: (error: string) => void;
  setLoading: (loading: boolean) => void;
}

/**
 * CounterView component for handling the counter functionality
 */
export function CounterView({ onError, setLoading }: CounterViewProps) {
  const [count, setCount] = useState<bigint>(BigInt(0));
  const { backendActor, isAuthenticated } = useBackendActors();

  // Create backend service with actor
  const backendService = createBackendService(backendActor, isAuthenticated);

  const fetchCount = async () => {
    if (!isAuthenticated) {
      onError("Please sign in to access your counter");
      return;
    }

    try {
      setLoading(true);
      const res = await backendService.getCount();
      setCount(res);
    } catch (err) {
      console.error(err);
      onError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const incrementCounter = async () => {
    if (!isAuthenticated) {
      onError("Please sign in to access your counter");
      return;
    }

    try {
      setLoading(true);
      const res = await backendService.incrementCounter();
      setCount(res);
    } catch (err) {
      console.error(err);
      onError(String(err));
    } finally {
      setLoading(false);
    }
  };

  // Fetch the initial count when component mounts and user is authenticated
  useEffect(() => {
    if (isAuthenticated) {
      fetchCount();
    }
  }, [isAuthenticated]);

  return (
    <Card title={`Counter: ${count.toString()}`}>
      <Button onClick={incrementCounter}>Increment</Button>
      <Button onClick={fetchCount}>Refresh Count</Button>
    </Card>
  );
}
