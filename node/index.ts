import * as net from 'net';

const server = net.createServer((connection) => {
  connection.setEncoding('utf8');

  let buffer: string[] = [];
  connection.on('data', (data: string) => {
    const [current, ...remainingLines] = data.split('\n');
    if (remainingLines.length > 0) {
      connection.write([...buffer, current, '\n'].join(''));
      buffer = remainingLines;
    } else {
      buffer.push(current);
    }
  });
});

server.listen(3000, () => {
  console.log('server listening on :3000');
});

