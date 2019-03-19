open Unix

let rec echo_line cin cout =
  let client_message = input_line cin in
  Printf.fprintf cout "%s\n" client_message;
  flush cout;
  if String.length client_message > 0 then
    echo_line cin cout

let handle_client s =
  let cout = out_channel_of_descr s
  and cin = in_channel_of_descr s in
  echo_line cin cout;
  close s

let rec accept_loop sock =
  let (client, _) = accept sock in
  let _ = Thread.create handle_client client in
  accept_loop sock

let () =
  let sock = socket PF_INET SOCK_STREAM 0 in
  setsockopt sock SO_REUSEADDR true;
  bind sock (ADDR_INET (inet_addr_of_string "127.0.0.1", 3000));
  print_endline "server listening on :3000";
  listen sock 24;
  accept_loop sock
