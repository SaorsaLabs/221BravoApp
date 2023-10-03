use crate::custom_types::MixerLink;


pub fn test_mixer_data () -> Vec<MixerLink> {
    let mut data: Vec<MixerLink> = Vec::new();

    // 0
    data.push(
        MixerLink{
            id: "ac1".to_string(),
            from: 100,
            level: 1,
            text: "test data".to_string(),
        });

    // 1
    data.push(
        MixerLink{
            id: "ac1".to_string(),
            from: 1541,
            level: 1,
            text: "test data".to_string(),
        });

    // 2
    data.push(
        MixerLink{
            id: "ac2".to_string(),
            from: 1501,
            level: 1,
            text: "test data".to_string(),
        });

    // 3
    data.push(
        MixerLink{
            id: "ac3".to_string(),
            from: 100,
            level: 1,
            text: "test data 1".to_string(),
        });

    // 4
    data.push(
        MixerLink{
            id: "ac3".to_string(),
            from: 100,
            level: 2,
            text: "test data 2".to_string(),
        });
    
    return data;
}
    
