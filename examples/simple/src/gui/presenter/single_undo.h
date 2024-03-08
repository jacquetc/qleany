// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "simple_example_presenter_export.h"
#include <QAction>
#include <QObject>

namespace Simple::Presenter
{
class SIMPLE_EXAMPLE_PRESENTER_EXPORT SingleUndo : public QObject
{
    Q_OBJECT

  public:
    explicit SingleUndo(QObject *parent = nullptr);

    bool enabled() const;
    QString text() const;

  public Q_SLOTS:
    void undo();

  Q_SIGNALS:
    void enabledChanged();
    void textChanged();

  private:
    QAction *m_action;
    bool m_enabled;
    QString m_text;
};

} // namespace Simple::Presenter