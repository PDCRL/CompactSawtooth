#include "Graph.h"
using std::vector;
using namespace std;

/***********************************************************
 Fine-grain Function used to create and add AU node in graph
************************************************************/

void Graph::add_node(int A_ID, int ts_AU, Graph_Node **ref)
{
	Graph_Node *pred = verHead, *curr;
	findVWind(&pred, &curr, A_ID);
//	cout<<curr->AU_ID<<endl;
//	cout<<A_ID<<"---"<<curr->AU_ID<<endl;

	//if vertex node is not in the grpah
	if(curr->AU_ID == A_ID){*ref = curr; curr->marked=false; return;}
	if(curr->AU_ID!= A_ID)
	{
		//cout<<A_ID<<endl;
		Graph_Node *temp     = new Graph_Node;
		temp->AU_ID          = A_ID;
		temp->ts             = ts_AU;
		temp->in_count       = 0;
		temp->edgeHead->next = temp->edgeTail;//empty edge list

		temp->next.store(curr, memory_order_seq_cst);

		//!!CAS operation if(pred->next == curr) 
		//!!then CAS successful and it sets pred->next = temp;

		while(atomic_compare_exchange_strong(
								 &(pred->next), &curr, temp ) != true)
		{
			findVWind(&pred, &curr, A_ID);

			if(curr->AU_ID == A_ID) 
			{
				*ref = curr;
				return;//if some other thread added this AU_ID vertex node 
			}
			temp->next.store(curr, memory_order_seq_cst);
			*ref = curr;
		}
		*ref = temp;
	}
}

/********************
 find vertex in graph
********************/
void Graph::findVWind(Graph_Node **ver_pre, Graph_Node **ver_curr, int key)
{
	Graph_Node *pre = *ver_pre, *curr = pre->next;

	while(curr != verTail && key != curr->AU_ID)
	{
		pre  = curr;
		curr = curr->next;
	}
	*ver_pre  = pre;
	*ver_curr = curr;
}



/************************************************
 Logic to add edges between atomic unit in graph.
************************************************/
void Graph::add_edge(int from, int to, int from_ts, int to_ts )
{
	Graph_Node *fromRef, *toRef;
	//if vertex node "from" is not present in graph, add "from" node
	add_node(from, from_ts, &fromRef);
	
	//if vertex node "to" is not present in graph, add "to" node
	add_node(to, to_ts, &toRef);

	Graph_Node *ver_pre = verHead, *ver_curr;
	
	// find "from" node in Grpah; ver_curr points to "from" node
	//findVWind(&ver_pre, &ver_curr, from);

	//optimization
	ver_curr = fromRef;
	
	//!check for duplicate Edge in "from" vertex edge list
	EdgeNode *edg_pre = ver_curr->edgeHead, *edg_curr,
							 *vcurrEdgTail = ver_curr->edgeTail;
	findEWind(vcurrEdgTail, &edg_pre, &edg_curr, to);
	
	
	if(edg_curr->au_id == to) 
		return;//edge added eariler

	//!--------------------------
	//!Actuall creation of edge
	//!--------------------------

	//! find "to" vertex node in graph:: (1) to increment in_count (because
	//! edge go from->to); (2) to store ref to "to" node in EdgeNode
	Graph_Node *refCurr, *refPre = verHead;
	if(!refCurr->marked and !refPre->marked){
	//refCurr: point to "this.to" node
	//findVWind(&refPre, &refCurr, to);
	
	refCurr = toRef;
	
	//in-count is atomic variable
	refCurr->in_count++;

	//edg_pre = ver_curr->edgeHead;
	
	//find window to add edge in edge_list of current node (from);
	//findEWind(vcurrEdgTail, &edg_pre, &edg_curr, to);

	EdgeNode *temp_edg  = new EdgeNode;
	temp_edg->au_id     = to;
	temp_edg->ref       = refCurr;

	temp_edg->next.store(edg_curr, memory_order_seq_cst);

	while(atomic_compare_exchange_strong( 
							&(edg_pre->next), &edg_curr, temp_edg ) != true)
	{
		findEWind(vcurrEdgTail, &edg_pre, &edg_curr, to);

		//if some other thread added this AU_ID edge node
		if(edg_curr->au_id == to) 
			break;

		temp_edg->next.store(edg_curr, memory_order_seq_cst);
	}
	}

return;
}


/****************************************
 find window to add new edge in edge list
****************************************/
void Graph::findEWind(EdgeNode* edgeTail, 
				EdgeNode **edg_pre, EdgeNode **edg_curr, int key)
{
	EdgeNode *pre  = *edg_pre;
	EdgeNode *curr = pre->next;
	while(curr != edgeTail && key < curr->au_id)
	{
		pre   = curr;
		curr  = curr->next;
	}
	*edg_pre  = pre;
	*edg_curr = curr;
}


/**********************************************
 Return Number of Dependenices in Block Graph.
**********************************************/
int Graph::print_grpah()
{
	int totalIncount = 0;
//	cout<<"==============================================================\n";
//	cout<<"  Grpah Node | In-Degree | Time Stamp | Out Edges (AU_IDs)\n";
//	cout<<"==============================================================\n";
	Graph_Node* g_temp = verHead->next;

	while( g_temp != verTail )
	{
		EdgeNode *o_temp = g_temp->edgeHead->next;
//		cout<<"\t"+to_string(g_temp->AU_ID)+" \t   "
//			+to_string(g_temp->in_count)+"\t\t "+to_string(g_temp->ts)+" \t";

		totalIncount = totalIncount + g_temp->in_count;

		while(o_temp != g_temp->edgeTail)
		{
//			cout<< "-->"+to_string(o_temp->au_id)+"";
			o_temp = o_temp->next;
		}
//		cout<<endl;
		g_temp = g_temp->next;
	}
//	cout<<"==============================================================\n";
//	cout<<"=============================================\n";
//	cout<<"\nNum of Edges in Graph = "+to_string(totalIncount);
//	cout<<"=============================================\n";
return totalIncount;
}




/*******************************************************************
 Copy the Graph Generated by Miner for Multiple Validator Execution
*******************************************************************/
void Graph::copy_BG(Graph *tGraph)
{

	tGraph->verHead        = new Graph_Node;
	tGraph->verTail        = new Graph_Node;
	tGraph->verHead->AU_ID = -1;                // vertex initial sentianl node
	tGraph->verTail->AU_ID = 2147483647;        // vertex end sentinal node
	tGraph->verHead->next  = verTail;
			
	tGraph->verHead->edgeHead->au_id = -1;
	tGraph->verHead->edgeTail->au_id = 2147483647;
	tGraph->verHead->edgeHead->next  = verHead->edgeTail;
		
	tGraph->verTail->edgeHead->au_id = -1;
	tGraph->verTail->edgeTail->au_id = 2147483647;
	tGraph->verTail->edgeHead->next  = verTail->edgeTail;


	//Step-1: add all vertices of ORIGINAL BG into COPY GRAPH.
	Graph_Node* v_temp = verHead->next;//main BG graph by miner
	Graph_Node* traker = tGraph->verHead;
	while( v_temp != verTail )
	{
		int incont           = v_temp->in_count;
		Graph_Node *temp     = new Graph_Node;
		temp->AU_ID          = v_temp->AU_ID;
		temp->ts             = v_temp->ts;
		temp->in_count       = incont;
		temp->edgeHead->next = temp->edgeTail;//empty edge list
		temp->next           = tGraph->verTail;
		
		traker->next = temp;
		traker       = traker->next;
		v_temp       = v_temp->next;
	}


	//Step-2: Go to each VERTEX of ORIGINAL GRAPH to add respective CONFLICT 
    //EDGES and there REFERENCES based on new Copied Graph.
    v_temp = verHead->next;//main BG graph by miner
	traker = tGraph->verHead->next;
	while( v_temp != verTail )
	{
		EdgeNode *e_temp     = v_temp->edgeHead->next;
		EdgeNode *e_traker   = traker->edgeHead;
		while(e_temp != v_temp->edgeTail)
		{
			EdgeNode *temp_edg  = new EdgeNode;
			temp_edg->au_id     = e_temp->au_id;
			
			//Add A loop here to add proper refrences as done for optimzation.
//			temp_edg->ref       = e_temp->ref;//here is the mistake
			Graph_Node* ref_t = tGraph->verHead;
			while(ref_t != tGraph->verTail)
			{
				if(ref_t->AU_ID == e_temp->au_id) break;
				ref_t = ref_t->next;
			}
			if(ref_t->AU_ID == e_temp->au_id) 
				temp_edg->ref = ref_t;
			
			temp_edg->next      = traker->edgeTail;

			e_traker->next      = temp_edg;
			e_traker            = e_traker->next;
			e_temp              = e_temp->next;
		}
		traker = traker->next;
		v_temp = v_temp->next;
	}
}




/******************************************
 Print Grpah: Need refrence to Graph Head.
******************************************/
void Graph::print_BG(Graph *copiedG)
{
	cout<<"==============================================================\n";
	cout<<"  Grpah Node | In-Degree | Time Stamp | Out Edges (AU_IDs)\n";
	cout<<"==============================================================\n";
	Graph_Node* g_temp = copiedG->verHead->next;

	while( g_temp != copiedG->verTail)
	{
		cout<<"\t"+to_string(g_temp->AU_ID)+" \t   "
			+to_string(g_temp->in_count)+"\t\t "+to_string(g_temp->ts)+" \t";
		EdgeNode *o_temp = g_temp->edgeHead->next;
		while(o_temp != g_temp->edgeTail){ 
			cout<< "-->"+to_string(o_temp->au_id)+"";
			o_temp = o_temp->next;
		}
		cout<<endl;
		g_temp = g_temp->next;
	}
	cout<<"==============================================================\n";
}





string Graph::find_pred(Graph *copiedG, int AU_ID)
{
	Graph_Node* g_temp = copiedG->verHead->next;
	string pred_list="";

	while( g_temp != copiedG->verTail)
	{
		int pred_id = g_temp->AU_ID;
		EdgeNode *o_temp = g_temp->edgeHead->next;
		while(o_temp != g_temp->edgeTail){ 
			if(AU_ID == o_temp->au_id) { pred_list.append(to_string(pred_id)); 
				pred_list.append(" "); }
			o_temp = o_temp->next; }
		g_temp = g_temp->next;
	}
return pred_list;
}




int Graph::inDeg_zero(Graph *BG) {

	Graph_Node* g_temp = BG->verHead->next;
	while( g_temp != BG->verTail ) {

		if(g_temp->in_count == 0 && g_temp->marked==false ){
			//g_temp->marked=true;
			return g_temp->AU_ID;}
		g_temp = g_temp->next;
	}
return -1;
}



/*************************************************************
 Remove a Dependency Edge from Graph to make miner Malicious.
*************************************************************/
void Graph::remove_Edge(Graph *BG)
{
//	cout<<"==============================================================\n";
//	cout<<"  Grpah Node | In-Degree | Time Stamp | Out Edges (AU_IDs)\n";
//	cout<<"==============================================================\n";
	int to_rm_conf = 1;

	start:
	Graph_Node* g_temp = BG->verHead->next;
	bool flag = false;
	while( g_temp->AU_ID != to_rm_conf && g_temp != BG->verTail)
	{
		g_temp = g_temp->next;
	}
	
	if(g_temp->AU_ID != to_rm_conf)
	{
		to_rm_conf++;
//		cout<<"I'm trying to remove conflict from au_ID "<<to_rm_conf<<endl;
		goto start;
	}
	int AUID;
	if( g_temp != BG->verTail )
	{
//		cout<<"\t"+to_string(g_temp->AU_ID)+" \t   "
//			+to_string(g_temp->in_count)+"\t\t "+to_string(g_temp->ts)+" \t";
		EdgeNode *o_temp = g_temp->edgeHead->next;
		EdgeNode *t_temp = g_temp->edgeHead;
		EdgeNode *s_temp = NULL;
		while(o_temp != g_temp->edgeTail)
		{
			s_temp = t_temp;
			t_temp = o_temp;
			o_temp = o_temp->next;
		}
		if(s_temp != NULL && o_temp == g_temp->edgeTail)
		{
			//flag = true;
			AUID = t_temp->au_id;
			s_temp->next = o_temp;
		
			Graph_Node* ref_t = BG->verHead->next;
			while(ref_t != BG->verTail)
			{
				if(ref_t->AU_ID == AUID) break;
				ref_t = ref_t->next;
			}
			if(ref_t->AU_ID == AUID)
				ref_t->in_count = ref_t->in_count-1;
		}
//		t_temp->next = o_temp;
//		cout<<endl;
//		g_temp = g_temp->next;
	}
	if(flag == true) return;
	else
	{
		//to_rm_conf++;
		goto start;
	}
//	cout<<"==============================================================\n";
}



/**************************************************************************
 Remove a Dependency Edge from given AU in Graph to make miner Malicious.
**************************************************************************/
void Graph::remove_AU(Graph *BG, int AU_ID)
{

Graph_Node *pred = verHead, *curr;
findVWind(&pred, &curr, AU_ID);

EdgeNode *next = curr->edgeTail;


while(curr->edgeHead->next != next)
{
remove_AU_Edge(BG, AU_ID);
}

pred->next.store(curr->next, memory_order_seq_cst);

}
/**************************************************************************
 Remove a Dependency Edge from given AU in Graph to make miner Malicious.
**************************************************************************/
void Graph::remove_AU_Edge(Graph *BG, int AU_ID)
{
//	cout<<"==============================================================\n";
//	cout<<"  Grpah Node | In-Degree | Time Stamp | Out Edges (AU_IDs)\n";
//	cout<<"==============================================================\n";
	int to_rm_conf = AU_ID;

	start:
	Graph_Node* g_temp = BG->verHead->next;
	bool flag = false;
	while( g_temp->AU_ID != to_rm_conf && g_temp != BG->verTail)
	{
		g_temp = g_temp->next;
	}

	int AUID;
	if( g_temp != BG->verTail )
	{
//		cout<<"\t"+to_string(g_temp->AU_ID)+" \t   "
//			+to_string(g_temp->in_count)+"\t\t "+to_string(g_temp->ts)+" \t";
		EdgeNode *o_temp = g_temp->edgeHead->next;
		EdgeNode *t_temp = g_temp->edgeHead;
		EdgeNode *s_temp = NULL;
		while(o_temp != g_temp->edgeTail)
		{
			s_temp = t_temp;
			t_temp = o_temp;
			o_temp = o_temp->next;
		}
		if(s_temp != NULL && o_temp == g_temp->edgeTail)
		{
			flag = true;
			AUID = t_temp->au_id;
			s_temp->next = o_temp;
		
			Graph_Node* ref_t = BG->verHead->next;
			while(ref_t != BG->verTail)
			{
				if(ref_t->AU_ID == AUID) break;
				ref_t = ref_t->next;
			}
			if(ref_t->AU_ID == AUID)
				ref_t->in_count = ref_t->in_count-1;
		}
//		t_temp->next = o_temp;
//		cout<<endl;
//		g_temp = g_temp->next;
	}
	if(flag == true) return;
	else
	{
		to_rm_conf++;
		goto start;
	}
//	cout<<"==============================================================\n";
}

int Graph::concBinGen(Graph *BG, vector<int>& ListAUs) {
	int count = 0;
	Graph_Node* g_temp  = BG->verHead->next;
	while( g_temp != BG->verTail ) {
		if(g_temp->in_count != 0)
			count++;
		else if(g_temp->in_count == 0 && g_temp->edgeHead->next == g_temp->edgeTail)
			ListAUs.push_back(g_temp->AU_ID);

		g_temp  = g_temp->next;
	}
//	print_BG(BG);


	g_temp  = BG->verHead->next;
//	print_BG(BG);
	Graph_Node* g_temp2 = BG->verHead;
	while( g_temp != BG->verTail ) {
		if(g_temp->in_count == 0 && g_temp->edgeHead->next == g_temp->edgeTail)
		{
//			ListAUs.push_back(g_temp->AU_ID);
			g_temp2->next.store(g_temp->next, memory_order_seq_cst);
		}
		else {
//			count++;
			g_temp2 = g_temp;
		}
		g_temp = g_temp->next;
	}
//	print_BG(BG);

	return count;
}

/****************************************
 find window to add new edge in edge list
****************************************/
int Graph::findInDeg(Graph *BG,int key)
{
	int count = 0;
	Graph_Node* g_temp = BG->verHead->next;
	while( g_temp != BG->verTail ) 
	{
		if(g_temp->AU_ID == key )
		    {
			return g_temp->in_count;
			}
		g_temp = g_temp->next;
	}
		return -1;
}



