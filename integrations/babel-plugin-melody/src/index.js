const { compiler } = require("melodyc");

const MARKER = "melody";

module.exports = () => ({
  visitor: {
    TemplateLiteral(literal) {
      let hasMelodyMarker =
        literal.node.leadingComments?.find?.(
          (comment) => comment.value.trim() === MARKER
        ) !== undefined;
      if (hasMelodyMarker) {
        literal.node.leadingComments = literal.node.leadingComments.filter(
          (comment) => comment.value.trim() !== MARKER
        );
        literal.traverse({
          TemplateElement(element) {
            const source = compiler(element.node.value.raw.replace(MARKER, ""));
            element.replaceWithSourceString(JSON.stringify(source));
          },
        });
      }
    },
    StringLiteral(literal) {
      let hasMelodyMarker =
        literal.node.leadingComments?.find?.(
          (comment) => comment.value.trim() === MARKER
        ) !== undefined;

      if (hasMelodyMarker) {
        literal.node.leadingComments = literal.node.leadingComments.filter(
          (comment) => comment.value.trim() !== MARKER
        );
        const source = compiler(literal.node.value);
        literal.replaceWithSourceString(JSON.stringify(source));
      }
    },
  },
});
