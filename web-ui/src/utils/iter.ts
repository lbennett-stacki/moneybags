export const roundRobin = (
  index: number,
  inputList: string[] | Record<number, string>,
) => {
  const list = Array.isArray(inputList) ? inputList : Object.values(inputList);

  return list[index % list.length];
};

export const shuffle = (array: string[]) => {
  return [...array].sort(() => Math.random() - 0.5);
};
