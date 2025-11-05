type LoadingProps = {
  text?: string;
};

const style = {
  width: "100%",
  height: "100%",
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  minHeight: "80dvh",
};

export const Loading = ({ text = "Loading..." }: LoadingProps) => {
  return (
    <div style={style}>
      <p>{text}</p>
    </div>
  );
};
