require 'socket'

server = TCPServer.open(3000)
puts "server listening on :3000"
loop {
  socket = server.accept
  Thread.start(socket) do |sock|
    sock.each_line do |line|
      sock.puts(line)
    end
    socket.close
  end
}
