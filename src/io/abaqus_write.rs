use types::Triangulation;

use std::io::BufWriter;
use std::io::Write;

pub fn write_to_abaqus_format<W: Write>(mut buf: BufWriter<W>, triangulation: &Triangulation) {
    AbaqusWriter {
        writer: buf,
        triangulation: triangulation
    }.write();
    //write_header(&mut buf);
}

struct AbaqusWriter<'a, W: Write> {
    writer: BufWriter<W>,
    triangulation: &'a Triangulation,
}

impl<'a, W: Write> AbaqusWriter<'a, W> {
    fn write(mut self) {
        self.write_header();
        self.write_nodes();
        self.write_elements();
        self.write_elset();
        self.write_footer();
    }

    fn write_header(&mut self) {
        self.writer.write("*Part, name=PART-1\n".as_bytes());
    }

    fn write_nodes(&mut self) {
        self.writer.write("*Node\n".as_bytes());
        for i in 0..self.triangulation.nodes().len() {
            let node = &self.triangulation.nodes()[i];
            self.writer.write(format!("{},\t{},\t{}\n", i + 1, node.x, node.y).as_bytes());
        }
    }

    fn write_elements(&mut self) {
        self.writer.write("*Element, type=CPE3\n".as_bytes());
        for i in 0..self.triangulation.elements().len() {
            let element = &self.triangulation.elements()[i];
            self.writer.write(format!("{},\t{},\t{},\t{}\n", i + 1, element.index_a().0 + 1, element.index_b().0 + 1, element.index_c().0 + 1).as_bytes());
        }
    }

    fn write_elset(&mut self) {
        self.writer.write("*Elset, elset=M_1\n".as_bytes());

        let eles = &self.triangulation.elements();
        let mut written = 0;
        while written < eles.len() {
            if written > 0 {
                self.writer.write(",".as_bytes());
            }
            if written % 10 == 0 && written != 0 {
                self.writer.write("\n".as_bytes());
            }

            self.writer.write((written + 1).to_string().as_bytes());
            written += 1;
        }

        self.writer.write("\n".as_bytes());
    }

    fn write_footer(&mut self) {
        self.writer.write("*Solid Section, elset=M_1, material=M_1
1.,
*End Part
**
**
** ASSEMBLY
**
*Assembly, name=Assembly
**
*Instance, name=PART-1-1, part=PART-1
*End Instance
**
*End Assembly\n".as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;
    use std::io::{BufWriter};

    #[test]
    fn testing_bufwriter_and_string() {
        let mut nodes = vec![Point2::new(0., 0.), Point2::new(1., 0.), Point2::new(1., 1.), Point2::new(0., 1.)];
        let mut eles = vec![Triangle::new(&nodes, N2Index(0), N2Index(1), N2Index(2)), Triangle::new(&nodes, N2Index(0), N2Index(2), N2Index(3))];
        let triangulation = Triangulation::new_from_prebuilt_triangulation(nodes, eles);
        //let tr
        let mut s = String::new();

        write_to_abaqus_format(BufWriter::new(unsafe { s.as_mut_vec() }), &triangulation);

        let expected_file = "*Part, name=PART-1
*Node
1,	0,	0
2,	1,	0
3,	1,	1
4,	0,	1
*Element, type=CPE3
1,	1,	3,	2
2,	1,	4,	3
*Elset, elset=M_1
1,2
*Solid Section, elset=M_1, material=M_1
1.,
*End Part
**
**
** ASSEMBLY
**
*Assembly, name=Assembly
**
*Instance, name=PART-1-1, part=PART-1
*End Instance
**
*End Assembly
";

        assert_eq!(expected_file, s);
    }
}