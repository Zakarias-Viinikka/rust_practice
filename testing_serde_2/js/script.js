
function send_data_to_js(data_in_transport_mode) {
  window.dispatchEvent(new CustomEvent('data_back', { detail: data_in_transport_mode }));
}
