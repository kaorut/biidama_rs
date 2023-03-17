# biidama_rs
Biidama for Rust

---

$$
m_1v_1+m_2v_2=m_1v_1'+m_2v_2' \tag{1}
$$
$$
-k=\frac{v_2'-v_1'}{v_2-v_1} \tag{2}
$$

By manipulating (2),

$$
-kv_1+kv_2=v_1'-v_2' \tag{2'}
$$

Multiplying both sides of (2') by $m_2$,

$$
-km_2v_1+km_2v_2=m_2v_1'-m_2v_2' \tag{2''}
$$

Adding (1) and (2''),

$$
(m_1-km_2)v_1+(k+1)m_2v_2=(m_1+m_2)v_1'
$$

Solving for $v_1'$,

$$
v_1'=\frac{(m_1-km_2)v_1+(k+1)m_2v_2}{m_1+m_2}
$$

Similarly, solving for $v_2'$,

$$
v_2'=\frac{(m_2-km_1)v_2+(k+1)m_1v_1}{m_2+m_1}
$$
