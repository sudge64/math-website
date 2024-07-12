import React from "react";

interface Props {
  children: React.ReactNode;
  char: string;
  color?:
    | "primary"
    | "secondary"
    | "success"
    | "danger"
    | "warning"
    | "info"
    | "light"
    | "dark"
    | "link";
  onClick: (char: string) => void;
}

const Button = ({ children, char, onClick, color = "primary" }: Props) => {
  return (
    <button className={"btn btn-" + color} onClick={() => onClick(char)}>
      {children}
    </button>
  );
};

export default Button;
