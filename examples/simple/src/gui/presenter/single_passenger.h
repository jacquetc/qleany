#pragma once

#include <QObject>

class SinglePassenger : public QObject
{
    Q_OBJECT
  public:
    explicit SinglePassenger(QObject *parent = nullptr);

    int id() const;
    void setId(int newId);
    void resetId();

    QString name() const;
    void setName(const QString &newName);

  signals:

    void idChanged();

    void nameChanged();

  private:
    int m_id;
    QString m_name;

    Q_PROPERTY(int id READ id WRITE setId RESET resetId NOTIFY idChanged FINAL)
    Q_PROPERTY(QString name READ name WRITE setName NOTIFY nameChanged FINAL)
};
