const bubbleSort = (data) => {
  for (let i = 0; i < data.length; i++) {
    for (let j = 0; j < data.length - i - 1; i++) {
      let current = data[j];
      let next = data[j + 1];

      if (current > next) {
        [data[j + 1], data[j]] = [data[j], data[j + 1]];
      }
    }
  }
};

const partition = (data) => {
  let pivot = data.length - 1;
  let i = 0;

  for (let j = 0; j < pivot; j++) {
    if (data[j] < data[pivot]) {
      [data[i], data[j]] = [data[j], data[i]];
      i += 1;
    }
  }

  [data[i], data[pivot]] = [data[pivot], data[i]];
  return i;
};

const quickSort = (data) => {};

const Sorter = (type, data) => {
  const sorters = {
    bubble: bubbleSort,
    quick: quickSort
  };
  return sorters[type](data);
};
