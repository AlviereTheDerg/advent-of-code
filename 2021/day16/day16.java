
import java.io.File;
import java.util.Arrays;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.Scanner;

public class day16 {
    private static class Packet {
        int version;
        int type_id;
        Long literal_value;
        LinkedList<Packet> subpackets;

        public Packet() {
            version = Integer.parseInt(poll_n_bits(3), 2);
            type_id = Integer.parseInt(poll_n_bits(3), 2);
            subpackets = new LinkedList<>();

            if (type_id == 4) { // literal value
                StringBuilder literal_builder = new StringBuilder();
                String holder;
                do {
                    holder = poll_n_bits(5);
                    literal_builder.append(holder.substring(1));
                } while (holder.charAt(0) == '1');
                literal_value = Long.parseLong(literal_builder.toString(), 2);
                return;
            }
            if (poll_n_bits(1).equals("0")) { // next 15 bits represent total bit length of subpackets
                int packet_length = Integer.parseInt(poll_n_bits(15), 2);
                int expected_remaining = packet_data.size() - packet_length;
                while (packet_data.size() > expected_remaining)
                    subpackets.add(new Packet());
                assert packet_data.size() == expected_remaining : "Uh oh!";
            } else { /* poll_n_bits(1).equals("1") */ // next 11 bits represent number of subpackets
                int packet_count = Integer.parseInt(poll_n_bits(11), 2);
                for (int i = 0; i < packet_count; i++)
                    subpackets.add(new Packet());
            }
        }

        public int extract_cumulative_version() {
            return version + subpackets.stream().mapToInt(Packet::extract_cumulative_version).sum();
        }

        public String toString() {
            if (type_id == 4)
                return String.format("(%d-%d-%d)", version, type_id, literal_value);
            StringBuilder subpacket_strings = new StringBuilder();
            Iterator<Packet> iterator = subpackets.iterator();
            if (iterator.hasNext()) {
                subpacket_strings.append(iterator.next().toString());
                while (iterator.hasNext()) {
                    subpacket_strings.append(",");
                    subpacket_strings.append(iterator.next().toString());
                }
            }
            return String.format("(%d-%d-[%s])", version, type_id, subpacket_strings.toString());
        }
    }

    private static LinkedList<Character> packet_data;
    private static String poll_n_bits(int n) {
        StringBuilder assemble = new StringBuilder();
        for (int i = 0; i < n; i++) assemble.append(packet_data.poll());
        return assemble.toString();
    }

    public static void main(String[] args) {
        try {
            Scanner input_scanner = new Scanner(new File("2021/day16/day16.txt"));
            packet_data = new LinkedList<>();
            Arrays.stream(input_scanner.next().split(""))
                .map(x -> Integer.toBinaryString(Integer.parseInt(x, 16))) // convert to binary, leading 0s chopped
                .map(x -> String.format("%4s", x).replace(' ', '0')) // re-add leading 0s
                .forEach(x -> x.chars().mapToObj(y->(char)y) // for each string, convert to char stream
                        .forEach(y -> packet_data.add(y))); // add to packet data list
            input_scanner.close();
            
            Packet outermost_packet = new Packet();
            System.out.println(outermost_packet.extract_cumulative_version());
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
