#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24);
} aura_ringbuf SEC("maps");

SEC("xdp")
int aura_audit_prog(struct xdp_md *ctx) {
    // 1. ये चेक करेगा कि क्या पैकेट आ रहा है
    __u32 dummy_ip = 0;
    
    // 2. हम यहाँ एक 'Ping' का डेटा बफर में 'Force' करेंगे ताकि रस्ट को दिखे
    bpf_ringbuf_output(&aura_ringbuf, &dummy_ip, sizeof(dummy_ip), 0);
    
    return XDP_PASS; // पैकेट को जाने दो, बस हमें ऑडिट दे दो
}
char _license[] SEC("license") = "GPL";
