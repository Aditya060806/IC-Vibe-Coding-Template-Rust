import { useState } from "react";

// Import components and views
import { Loader, ErrorDisplay, Header, ViewType } from "./components";
import { GreetingView, CounterView, LlmPromptView } from "./views";

function App() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | undefined>();
  const [currentView, setCurrentView] = useState<ViewType>("greeting");
  const [isSignedIn, setIsSignedIn] = useState(false);

  const handleError = (errorMessage: string) => {
    setError(errorMessage);
  };

  const handleViewChange = (view: ViewType) => {
    // Only allow counter view if signed in
    if (view === "counter" && !isSignedIn) {
      return;
    }
    setCurrentView(view);
    setError(undefined); // Clear any existing errors when switching views
  };

  const handleSignInToggle = () => {
    const newSignedInState = !isSignedIn;
    setIsSignedIn(newSignedInState);

    // If signing out and currently on counter view, switch to greeting
    if (!newSignedInState && currentView === "counter") {
      setCurrentView("greeting");
    }
  };

  const renderCurrentView = () => {
    switch (currentView) {
      case "greeting":
        return <GreetingView onError={handleError} setLoading={setLoading} />;
      case "counter":
        return <CounterView onError={handleError} setLoading={setLoading} />;
      case "llm":
        return <LlmPromptView onError={handleError} setLoading={setLoading} />;
      default:
        return <GreetingView onError={handleError} setLoading={setLoading} />;
    }
  };

  return (
    <div className="min-h-screen bg-gray-800 text-white">
      {/* Header */}
      <Header
        currentView={currentView}
        onViewChange={handleViewChange}
        isSignedIn={isSignedIn}
        onSignInToggle={handleSignInToggle}
      />

      {/* Main Content */}
      <div className="mx-auto w-full max-w-4xl space-y-8 p-8">
        {/* Current View */}
        <div className="space-y-6">{renderCurrentView()}</div>

        {/* Loading and Error States */}
        {loading && !error && <Loader />}
        {!!error && <ErrorDisplay message={error} />}
      </div>
    </div>
  );
}

export default App;
