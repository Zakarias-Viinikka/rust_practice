// Restore the saved values when the page loads
document.getElementById('edit-row-id').value = localStorage.getItem('edit-row-id') || '';
document.getElementById('edit-col').value   = localStorage.getItem('edit-col')   || '';
document.getElementById('edit-val').value = localStorage.getItem('edit-val') || '';
//delete
document.getElementById('delete-row-id').value = localStorage.getItem('delete-row-id') || '';
