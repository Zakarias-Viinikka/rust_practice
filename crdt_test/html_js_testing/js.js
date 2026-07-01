var el = document.getElementById("sortable-container");
var sortable = Sortable.create(el, {
  handle: ".drag-handle",
  animation: 150,
  onEnd: function (evt) {
    var above = evt.item.previousElementSibling;
    var below = evt.item.nextElementSibling;

    if (below) {
      console.log("above:", below.querySelector("input").value);
    } else {
      console.log("above: (none)");
    }

    if (above) {
      console.log("below:", above.querySelector("input").value);
    } else {
      console.log("below: (none)");
    }
  },
});
