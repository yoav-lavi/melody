const regex = new RegExp(/*melody*/ `2 to 3 of "na";`);

const otherRegex = new RegExp(/*melody*/ '5 to 9 of "other";');

const rawRegex = /*melody*/ `
  <start>;
  "other";
  <end>;
`;

const thirdRegex = new RegExp(rawRegex);

/*
after babel:

const regex = new RegExp("(?:na){2,3}");
const otherRegex = new RegExp("(?:other){5,9}");
const rawRegex = "^other$";
const thirdRegex = new RegExp(rawRegex);
*/
