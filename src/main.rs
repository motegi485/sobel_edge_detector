mod edge_detection;

fn main() {
    let input_path = "original_images/edge_detection_test.jpg";

    let grayscale_img = edge_detection::convert_to_grayscale(input_path);
    grayscale_img
        .save("processed_images/grayscale.png")
        .expect("画像の保存に失敗しました");

    let edge_img = edge_detection::apply_sobel(grayscale_img);
    edge_img
        .save("processed_images/edge.png")
        .expect("画像の保存に失敗しました");
}
