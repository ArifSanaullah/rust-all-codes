mod teachers;
mod syllabus;

fn main() {
    syllabus::Bio_syll();
    syllabus::maths_syll();
    teachers::chem_teacher();
    teachers::maths_teacher();
    teachers::phy_teacher();
    teachers::bio_teacher();
}
