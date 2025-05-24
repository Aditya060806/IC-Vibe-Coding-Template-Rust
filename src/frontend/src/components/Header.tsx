import ReactIcon from "../../assets/React-icon.webp";

export type ViewType = "greeting" | "counter" | "llm";

interface HeaderProps {
  className?: string;
  currentView: ViewType;
  onViewChange: (view: ViewType) => void;
  isSignedIn: boolean;
  onSignInToggle: () => void;
}

export function Header({
  className = "",
  currentView,
  onViewChange,
  isSignedIn,
  onSignInToggle,
}: HeaderProps) {
  const logoStyle = {
    animation: "logo-spin 60s linear infinite",
  };

  const getButtonClassName = (view: ViewType) => {
    const baseClassName =
      "rounded px-3 py-2 text-sm font-medium transition-colors";
    return currentView === view
      ? `${baseClassName} bg-gray-600 text-white`
      : `${baseClassName} hover:bg-gray-700`;
  };

  return (
    <>
      <style>
        {`
          @keyframes logo-spin {
            from {
              transform: rotate(0deg);
            }
            to {
              transform: rotate(360deg);
            }
          }
        `}
      </style>
      <header className={`w-full bg-gray-800 text-white ${className}`}>
        <div className="mx-auto flex max-w-4xl items-center justify-between p-6">
          {/* Left side: Logo, Title, and Subtitle */}
          <div className="flex items-center space-x-6">
            <a href="https://reactjs.org" target="_blank" rel="noreferrer">
              <img
                src={ReactIcon}
                className="h-12 w-12 will-change-[filter] hover:drop-shadow-[0_0_2em_#61dafbaa] motion-reduce:animate-none"
                style={logoStyle}
                alt="React logo"
              />
            </a>
            <div>
              <h1 className="text-2xl font-bold">Vibe Coding Template</h1>
              <h2 className="text-sm text-gray-300">
                React + Rust + Internet Computer
              </h2>
            </div>
          </div>

          {/* Right side: Navigation */}
          <nav className="flex items-center space-x-6">
            <div className="flex space-x-4">
              <button
                onClick={() => onViewChange("greeting")}
                className={getButtonClassName("greeting")}
              >
                Greeting
              </button>
              {isSignedIn && (
                <button
                  onClick={() => onViewChange("counter")}
                  className={getButtonClassName("counter")}
                >
                  Counter
                </button>
              )}
              <button
                onClick={() => onViewChange("llm")}
                className={getButtonClassName("llm")}
              >
                LLM Prompt
              </button>
            </div>

            {/* Sign In/Out Button */}
            <button
              onClick={onSignInToggle}
              className="rounded bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
            >
              {isSignedIn ? "Sign Out" : "Sign In"}
            </button>
          </nav>
        </div>
      </header>
    </>
  );
}
