import yaml
from PySide6.QtWidgets import (
    QMainWindow,
    QGraphicsView,
    QGraphicsScene,
    QGraphicsRectItem,
    QGraphicsTextItem,
    QGraphicsLineItem,
)
from PySide6.QtCore import Qt, QRectF
from PySide6.QtGui import QPen, QColor, QFont


class EntityRelationshipWindow(QMainWindow):
    def __init__(self, file_path, parent=None):
        super(EntityRelationshipWindow, self).__init__(parent)
        self.setWindowTitle("Entity Relationship Diagram")

        self.view = QGraphicsView(self)
        self.scene = QGraphicsScene()
        self.view.setScene(self.scene)

        self.entity_items = {}
        self.load_yaml(file_path)

        # set the window size to the scene size but not less than 800x600 and not more than 1920x1080
        self.setFixedSize(self.scene.width() + 20, self.scene.height() + 20)
        self.setMinimumSize(800, 600)
        self.setMaximumSize(1920, 1080)

        self.setCentralWidget(self.view)

    def load_yaml(self, file_path):
        with open(file_path, "r") as f:
            data = yaml.load(f, Loader=yaml.FullLoader)

        entities = data["entities"]["list"]

        x, y = 0, 0

        for entity in entities:
            rect_height = 50 + len(entity["fields"]) * 20
            rect = QGraphicsRectItem(QRectF(x, y, 200, rect_height))
            rect.setZValue(1)
            rect.setBrush(QColor("lightgray"))
            rect.setPen(QPen(Qt.black))
            self.scene.addItem(rect)

            title = f"<b>{entity['name']}</b>"
            entity_name = QGraphicsTextItem(rect)
            entity_name.setHtml(title)
            entity_name.setPos(x + 10, y + 10)
            self.scene.addItem(entity_name)

            y_field = y + 30
            for field in entity["fields"]:
                field_text = f"{field['name']} : {field['type']}"
                field_item = QGraphicsTextItem(field_text, rect)
                field_item.setPos(x + 10, y_field)
                field_item.setZValue(2)
                field_item.setDefaultTextColor(QColor("black"))
                field_item.setFont(QFont("Arial", 10))

                self.scene.addItem(field_item)
                y_field += 20

            self.entity_items[entity["name"]] = rect

            y += rect_height + 50

        for entity in entities:
            parent = entity.get("parent")
            if parent and parent != "EntityBase":
                parent_item = self.entity_items[parent]
                child_item = self.entity_items[entity["name"]]
                x1, y1 = parent_item.rect().center().x(), parent_item.rect().bottom()
                x2, y2 = child_item.rect().center().x(), child_item.rect().top()

                line_type = (
                    Qt.SolidLine
                    if any(field.get("strong", False) for field in entity["fields"])
                    else Qt.DotLine
                )
                line = QGraphicsLineItem(x1, y1, x2, y2)
                pen = QPen()
                pen.setStyle(line_type)
                line.setPen(pen)
                line.setZValue(0)
                self.scene.addItem(line)
