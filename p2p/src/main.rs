use libp2p::{identity, PeerId};
use libp2p::futures::StreamExt;
use libp2p::swarm::{DummyBehaviour, Swarm, SwarmEvent};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 密钥对的类型是 25519
    let new_key = identity::Keypair::generate_ed25519();
    /*
        使用密钥对的公钥(new_key.public())生产 PeerId
        在 libp2p 中，公钥不会用来直接验证 Peer 的身份，
        我们使用的是它的一个 hash 的版本，也就是这个 peer_id
     */
    let new_peer_id = PeerId::from(new_key.public());
    println!("New Peer ID is {:?}", new_peer_id);

    // 创建一个空的网络行为，这个行为会关联到 swarm
    let behaviour = DummyBehaviour::default();
    // 创建一个传输
    let transport = libp2p::development_transport(new_key).await?;
    let mut swarm = Swarm::new(transport, behaviour, new_peer_id);
    // 让 swarm 监听 0.0.0.0 地址，端口是 0，
    // 0.0.0.0 表示本地机器上所有的 ipv4 的地址
    // 端口 0 指随机选一个端口
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;


    /*
        持续的轮询来检查事件
     */
    loop {
        match swarm.select_next_some().await {
            /*
                如果是 SwarmEvent::NewListenAddr 事件，
                即创建一个新的监听地址
             */
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on Local Address {:?}", address);
            }
            _ => {}
        }
    }

}
