package io.quiche4j.examples;

import java.io.IOException;
import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetAddress;
import java.net.SocketTimeoutException;
import java.net.UnknownHostException;
import java.net.URI;
import java.net.URISyntaxException;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;

import io.quiche4j.Config;
import io.quiche4j.Connection;
import io.quiche4j.H3Config;
import io.quiche4j.H3Connection;
import io.quiche4j.H3PollEvent;
import io.quiche4j.Header;
import io.quiche4j.Quiche;

public class H3Client {

    public static final int MAX_DATAGRAM_SIZE = 1350;

    public static void main(String[] args) throws UnknownHostException, IOException {
        if(0 == args.length) {
            System.out.println("Usage: ./h3-client.sh <URL>");
            System.exit(1);
        }

        final String url = args[0];
        URI uri;
        try {
            uri = new URI(url);
            System.out.println("> sending request to " + uri);
        } catch (URISyntaxException e) {
            System.out.println("Failed to parse URL " + url);
            System.exit(1);
            return;
        }

        final int port = uri.getPort();
        final InetAddress address = InetAddress.getByName(uri.getHost());

        final Config config = Config.newConfig(Quiche.PROTOCOL_VERSION);
        // CAUTION: this should not be set to `false` in production
        config.verityPeer(false);

        config.setApplicationProtos(Quiche.H3_APPLICATION_PROTOCOL);
        config.setMaxIdleTimeout(5000);
        config.setMaxUdpPayloadSize(MAX_DATAGRAM_SIZE);
        config.setInitialMaxData(10_000_000);
        config.setInitialMaxStreamDataBidiLocal(1_000_000);
        config.setInitialMaxStreamDataBidiRemote(1_000_000);
        config.setInitialMaxStreamDataUni(1_000_000);
        config.setInitialMaxStreamsBidi(100);
        config.setInitialMaxStreamsUni(100);
        config.setDisableActiveMigration(true);

		final byte[] connId = Quiche.newConnectionId();
        final Connection conn = Quiche.connect(uri.getHost(), connId, config);

        final byte[] buffer = new byte[MAX_DATAGRAM_SIZE];
        final int handshakeLength = conn.send(buffer);
		System.out.println("> handshake size: " + handshakeLength);

		final DatagramPacket handshakePacket = new DatagramPacket(
			buffer, handshakeLength, address, port);
        final DatagramSocket socket = new DatagramSocket(10002);
        socket.setSoTimeout(1000);
		socket.send(handshakePacket);

		while(!conn.isEstablished() && !conn.isClosed()) {
			DatagramPacket packet = new DatagramPacket(buffer, buffer.length);
			socket.receive(packet);
			final int recvBytes = packet.getLength();
			System.out.println("> socket.recieve " + recvBytes + " bytes");
			final int read = conn.recv(Arrays.copyOfRange(packet.getData(), 0, recvBytes));
			System.out.println("> conn.recv " + read + " bytes");
		}

        final H3Config h3Config = H3Config.newConfig();
		final H3Connection h3Conn = H3Connection.withTransport(conn, h3Config);
        List<Header> req = new ArrayList<Header>();
        req.add(new Header(":method", "GET"));
        req.add(new Header(":scheme", uri.getScheme()));
        req.add(new Header(":authority", uri.getAuthority()));
        req.add(new Header(":path", uri.getPath()));
        req.add(new Header("user-agent", "quiche4j"));
		h3Conn.sendRequest(req, true);

		System.out.println("> started sending cycle");
		while(true) {
            int payloadLength = conn.send(buffer);
			if (-1 == payloadLength || 0 == payloadLength) break;
			System.out.println("> h3.send "+ payloadLength + " bytes");
			DatagramPacket packet = new DatagramPacket(
				buffer, payloadLength, address, port);
			socket.send(packet);
		}
        System.out.println("> request succesfully sent");

        final AtomicBoolean reading = new AtomicBoolean(false);
		while(!conn.isClosed()) {
            while(reading.get()) {
                // READING
                DatagramPacket packet = new DatagramPacket(buffer, buffer.length);
                try {
                    socket.receive(packet);
                    final int recvBytes = packet.getLength();
                    System.out.println("> socket.recieve " + recvBytes + " bytes");
                    final int read = conn.recv(Arrays.copyOfRange(packet.getData(), 0, recvBytes));
                    System.out.println("> h3.recv " + read + " bytes");
                } catch (SocketTimeoutException e) {
                    conn.onTimeout();
                    reading.set(false);
                }

                // POLL
                final Long streamId = h3Conn.poll(new H3PollEvent() {
                    public void onHeader(long streamId, String name, String value) {
                        System.out.println(name + ": " + value);
                    }

                    public void onData(long streamId) {
                        final byte[] body = new byte[MAX_DATAGRAM_SIZE];
                        final int bodyLength = h3Conn.recvBody(streamId, body);
                        System.out.println("> get body " + bodyLength + " bytes");
                        final byte[] buf = Arrays.copyOfRange(body, 0, bodyLength);
                        System.out.println(new String(buf, StandardCharsets.UTF_8));
                    }

                    public void onFinished(long streamId) {
                        System.out.println("> response finished");
                        System.out.println("> close code " + conn.close(true, 0x00, "kthxbye"));
                        reading.set(false);
                    }
                });

                if(null == streamId) reading.set(false);
            }

            // WRITING
            int payloadLength = conn.send(buffer);
            if (-1 == payloadLength || 0 == payloadLength) {
                reading.set(true);
                continue;
            }
            System.out.println("> h3.send "+ payloadLength + " bytes");
			DatagramPacket packet = new DatagramPacket(buffer, payloadLength, address, port);
            socket.send(packet);
        }

        System.out.println("> conn is closed");
        System.out.println(conn.stats());
        socket.close();
    }
}
