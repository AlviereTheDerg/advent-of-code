
import java.io.File;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.LinkedList;
import java.util.Scanner;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class day19 {
    private static class Coordinate {
        public int x, y, z;

        public Coordinate(String input_string) {
            String[] buffer = input_string.split(",");
            x = Integer.parseInt(buffer[0]);
            y = Integer.parseInt(buffer[1]);
            z = Integer.parseInt(buffer[2]);
        }

        public Coordinate(int x, int y, int z) {
            this.x = x;
            this.y = y;
            this.z = z;
        }

        public Coordinate add(Coordinate that) {
            return new Coordinate(this.x + that.x, this.y + that.y, this.z + that.z);
        }
        public Coordinate sub(Coordinate that) {
            return new Coordinate(this.x - that.x, this.y - that.y, this.z - that.z);
        }

        public int square_magnitude() {
            return x*x + y*y + z*z;
        }

        public int square_distance(Coordinate that) {
            return this.sub(that).square_magnitude();
        }

        public String toString() {
            return String.format("(%d,%d,%d)", x,y,z);
        }

        public int hashCode() {
            return (x << 20) + (y << 10) + z;
        }
        public boolean equals(Object obj) {
            if (!(obj instanceof Coordinate)) return false;
            Coordinate that = (Coordinate) obj;
            return this.x == that.x && this.y == that.y && this.z == that.z;
        }

        public Coordinate rotate(int x_ticks, int y_ticks, int z_ticks) {
            int rot_x = this.x, rot_y = this.y, rot_z = this.z;
            int buffer;
            for (int i = 0; i < x_ticks; i++) {
                buffer = rot_y;
                rot_y = rot_z;
                rot_z = -buffer;
            }
            for (int i = 0; i < y_ticks; i++) {
                buffer = rot_z;
                rot_z = rot_x;
                rot_x = -buffer;
            }
            for (int i = 0; i < z_ticks; i++) {
                buffer = rot_x;
                rot_x = rot_y;
                rot_y = -buffer;
            }
            return new Coordinate(rot_x, rot_y, rot_z);
        }
        public Coordinate rotate(Coordinate orientation) {
            return this.rotate(orientation.x, orientation.y, orientation.z);
        }
        public Coordinate get_relative_rotation(Coordinate anchor) {
            for (int x = 0; x < 4; x++)
                for (int y = 0; y < 4; y++)
                    for (int z = 0; z < 4; z++) {
                        Coordinate rotation = this.rotate(x,y,z);
                        if (anchor.equals(rotation))
                            return new Coordinate(x,y,z);
                    }
            System.out.println(String.format("Couldn't align %s to %s", this.toString(), anchor.toString()));
            return null;
        }
    }

    private static class Edge {
        Coordinate a, b;
        int sq_dist;

        public Edge(Coordinate a, Coordinate b) {
            this.a = a; this.b = b;
            this.sq_dist = a.square_distance(b);
        }

        public boolean contains(Coordinate coord) {
            return a.equals(coord) || b.equals(coord);
        }
        public Coordinate other_end_of(Coordinate end) {
            if (!this.contains(end)) return null;
            return this.a.equals(end) ? this.b : this.a;
        }

        public int hashCode() {
            return sq_dist;
        }
        public boolean equals(Object obj) {
            return (obj instanceof Edge) && ((Edge) obj).sq_dist == this.sq_dist;
        }

        public String toString() {
            return String.format("[length %d between %s and %s]", sq_dist, a.toString(), b.toString());
        }
    }

    private static class Detector {
        List<Coordinate> beacons;
        HashSet<Edge> edges;
        Coordinate position, orientation;
        int id;
        public static int overlap_count = 3;

        public Detector(String raw_input) {
            String[] split_input = raw_input.split("\n");

            // get the beacon id
            Matcher id_extractor = Pattern.compile("^--- scanner (\\d+) ---$").matcher(split_input[0]);
            id_extractor.find();
            id = Integer.parseInt(id_extractor.group(1));

            beacons = new ArrayList<>();
            for (int i = 1; i < split_input.length; i++)
                beacons.add(new Coordinate(split_input[i]));

            edges = new HashSet<>();
            for (int i = 0; i < beacons.size(); i++)
                for (int j = i+1; j < beacons.size(); j++)
                    edges.add(new Edge(beacons.get(i), beacons.get(j)));
            if (edges.size() != (beacons.size() * (beacons.size() - 1))/2)
                System.out.println("Uh oh!");
        }

        public boolean shares_overlap(Detector other) {
            HashSet<Edge> intersection = new HashSet<>(other.edges);
            intersection.retainAll(this.edges);
            return intersection.size() >= ((overlap_count * (overlap_count-1))/2);
        }

        public String toString() {
            return String.format("%d: %s", id, beacons.toString());
        }

        private Edge[] get_matching_edge(Detector anchor) {
            HashSet<Edge> anchored_overlap = new HashSet<>(anchor.edges), unanchored_overlap = new HashSet<>(this.edges);
            anchored_overlap.retainAll(unanchored_overlap);
            unanchored_overlap.retainAll(anchored_overlap);
            System.out.println(String.format("Detectors %d and %d overlap %d and %d edges", this.id, anchor.id, unanchored_overlap.size(), anchored_overlap.size()));

            Edge anchored_edge_a = anchored_overlap.iterator().next();
            Edge unanchored_edge_a = unanchored_overlap.stream().filter(edge -> edge.sq_dist == anchored_edge_a.sq_dist).toList().get(0);

            Coordinate anchored_coordinate_a = anchored_edge_a.a, anchored_coordinate_b = anchored_edge_a.b;
            Edge anchored_edge_b = anchored_overlap.stream().filter(edge -> edge.contains(anchored_coordinate_a) && edge.sq_dist != anchored_edge_a.sq_dist).toList().get(0);
            List<Edge> sample_A = unanchored_overlap.stream().filter(edge -> edge.sq_dist == anchored_edge_b.sq_dist).toList();
            if (sample_A.size() != 1) System.out.println("UH OH!");
            Edge unanchored_edge_b = sample_A.get(0);
            Coordinate unanchored_coordinate = (unanchored_edge_b.contains(unanchored_edge_a.a)) ? unanchored_edge_a.a : unanchored_edge_a.b;

            Edge constructed_anchored_edge = new Edge(anchored_coordinate_a.rotate(anchor.orientation).add(anchor.position), anchored_coordinate_b.rotate(anchor.orientation).add(anchor.position));
            System.out.println(String.format("Anchor edge %s becoming %s", anchored_edge_a.toString(), constructed_anchored_edge.toString()));
            Edge constructed_unanchored_edge = new Edge(unanchored_coordinate, unanchored_edge_a.other_end_of(unanchored_coordinate));
            return new Edge[]{constructed_anchored_edge, constructed_unanchored_edge};
        }

        public void align_to(Detector anchor) {
            if (!this.shares_overlap(anchor)) throw new IllegalArgumentException("Detectors do not share overlap");
            System.out.println(String.format("Scanner %d anchoring to %d", this.id, anchor.id));
            Edge[] matching_edges = this.get_matching_edge(anchor);
            System.out.println(String.format("Aligning anchor edge %s to edge %s", matching_edges[0].toString(), matching_edges[1].toString()));

            Edge anchored_edge = matching_edges[0], unanchored_edge = matching_edges[1];
            Coordinate anchored_coordinate = anchored_edge.a, unanchored_coordinate = unanchored_edge.a;

            Coordinate anchor_diff = anchored_coordinate.sub(anchored_edge.other_end_of(anchored_coordinate));
            Coordinate unanchor_diff = unanchored_coordinate.sub(unanchored_edge.other_end_of(unanchored_coordinate));

            System.out.println(anchor_diff);
            System.out.println(unanchor_diff);

            Coordinate unanchor_orientation = unanchor_diff.get_relative_rotation(anchor_diff);
            System.out.println(unanchor_orientation);
            if (unanchor_orientation == null)
                throw new IllegalArgumentException(String.format("Anchor diff %s, unanchor diff %s", anchor_diff.toString(), unanchor_diff.toString()));

            Coordinate alt_anchored_coordinate = anchored_coordinate;
            Coordinate alt_unanchored_coordinate = unanchored_coordinate.rotate(unanchor_orientation);
            Coordinate unanchor_position = alt_anchored_coordinate.sub(alt_unanchored_coordinate);
            this.set_alignment(unanchor_position, unanchor_orientation);
            Set<Coordinate> test = anchor.get_aligned_coords().stream().collect(Collectors.toSet());
            test.retainAll(this.get_aligned_coords());
            System.out.println(test.size());
        }

        public void set_alignment(Coordinate position, Coordinate orientation) {
            this.position = position;
            this.orientation = orientation;
        }

        public List<Coordinate> get_aligned_coords() {
            return beacons.stream().map(beacon -> beacon.rotate(orientation).add(position)).toList();
        }
    }

    public static void main(String[] args) {
        try {
            File input_file = new File("2021/day19/day19.txt");
            Scanner input_scanner = new Scanner(input_file);
            input_scanner.useDelimiter("\n\n");

            List<Detector> scanners = new ArrayList<>();
            while (input_scanner.hasNext())
                scanners.add(new Detector(input_scanner.next()));
            input_scanner.close();

            HashSet<Detector> anchored = new HashSet<>();
            HashSet<Detector> unanchored = new HashSet<>(scanners);
            LinkedList<Detector> anchoring = new LinkedList<>();
            Detector anchor = scanners.get(0);
            anchor.set_alignment(new Coordinate(0,0,0), new Coordinate(0,0,0));
            unanchored.remove(anchor); anchored.add(anchor); anchoring.add(anchor);

            while (anchoring.size() > 0) {
                Iterator<Detector> iter = unanchored.iterator();
                while (iter.hasNext()) {
                    Detector entry = iter.next();
                    if (!entry.shares_overlap(anchoring.getFirst())) continue;
                    iter.remove();
                    anchoring.addLast(entry);
                    anchored.add(entry);
                    entry.align_to(anchoring.getFirst());
                    System.out.println();
                }
                anchoring.pop();
            }
            HashSet<Coordinate> beacons = new HashSet<>();
            for (Detector hehe : scanners)
                beacons.addAll(hehe.get_aligned_coords());
            System.out.println(beacons.size());
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
